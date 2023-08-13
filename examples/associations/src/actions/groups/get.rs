use std::collections::HashSet;

use crate::{
    actions::ExecuteAction,
    context::AppState,
    models::{
        user::{User, UserId},
        user_group::{UserGroup, UserGroupId},
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct UserGroupDetails {
    pub user_group: UserGroup,
    pub users: Vec<User>,
}

pub struct GetUserGroup {
    user_group_id: UserGroupId,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GetUserGroupError {
    GroupDoesNotExist,
}

impl ExecuteAction<AppState> for GetUserGroup {
    type Output = Result<UserGroupDetails, GetUserGroupError>;

    fn execute(self, ctx: &mut AppState) -> Self::Output {
        let user_group = ctx
            .user_groups
            .iter()
            .find(|ug| ug.id == self.user_group_id)
            .cloned()
            .ok_or_else(|| GetUserGroupError::GroupDoesNotExist)?;
        let user_ids: HashSet<UserId> = ctx
            .user_in_groups
            .iter()
            .filter_map(|uig| {
                if uig.user_group_id == self.user_group_id {
                    Some(uig.user_id)
                } else {
                    None
                }
            })
            .collect();
        let mut users: Vec<User> = ctx
            .users
            .iter()
            .filter(|u| user_ids.contains(&u.id))
            .cloned()
            .collect();
        users.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(UserGroupDetails { user_group, users })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::context::{TestContext, TestContextFabriko};
    use fabriko::WithRelatedResources;
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn should_get_user_group_details() {
        let state = Rc::new(RefCell::new(AppState::default()));
        let mut f = TestContextFabriko::new(TestContext::new(state.clone()));

        let alice = f.user(|u| u.name("Alice"));
        let bob = f.user(|u| u.name("Bob"));
        // Cedric is not a part of the group. We add him to check for unwanted side-effects.
        let _cedric = f.user(|u| u.name("Cedric"));
        let (ug, _) = f.user_group(|ug| {
            ug.name("My user group").with_related_resources(|ug| {
                ug.with_user_in_group(|uig| uig.user_id(alice.id))
                    .with_user_in_group(|uig| uig.user_id(bob.id))
            })
        });

        let ug_details = GetUserGroup {
            user_group_id: ug.id,
        }
        .execute(&mut state.borrow_mut());

        let (user_group, users) = match ug_details {
            Ok(UserGroupDetails { user_group, users }) => (user_group, users),
            Err(e) => panic!("Unexpected error {e:?}"),
        };
        assert_eq!(user_group, ug);
        assert_eq!(users, vec![alice, bob]);
    }

    #[test]
    fn should_fail_to_get_user_group_that_does_not_exist() {
        let state = Rc::new(RefCell::new(AppState::default()));
        let mut f = TestContextFabriko::new(TestContext::new(state.clone()));

        // This is an irrelevant user group that we inserted to check for side effects
        let _ug = f.user_group(|ug| ug);

        let result = GetUserGroup {
            user_group_id: UserGroupId::new(0xDEAD),
        }
        .execute(&mut state.borrow_mut());

        assert_eq!(result, Err(GetUserGroupError::GroupDoesNotExist));
    }
}
