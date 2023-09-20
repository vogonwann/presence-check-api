use diesel::prelude::*;
use diesel::{SqliteConnection, QueryResult};
use crate::models::{NewUser, User};
use crate::schema::users;

pub struct UserRepository;
impl UserRepository {
    pub fn get_all(connection: &mut SqliteConnection) -> QueryResult<Vec<User>> {
        users::table
            .order(users::id.desc())
            .load::<User>(connection)
    }

    pub fn get_by_id(connection: &mut SqliteConnection, id: i32) -> QueryResult<User> {
        users::table
            .find(id)
            .get_result::<User>(connection)
    }

    pub fn create(connection: &mut SqliteConnection, new_user: NewUser) -> QueryResult<User>{
        let active_user: NewUser = NewUser { name: new_user.name, last_name: new_user.last_name, is_active: 1 };
        let _ = diesel::insert_into(users::table)
            .values(active_user)
            .execute(connection);

        let last_id = Self::last_inserted_id(connection)?;
        Self::get_by_id(connection, last_id)
    }

    pub fn update(connection: &mut SqliteConnection, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(user.id))
            .set((
                users::name.eq(&user.name.to_owned()),
                users::last_name.eq(&user.last_name.to_owned())
            ))
                .execute(connection)?;

        Self::get_by_id(connection, user.id)
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id))
            .execute(connection)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        users::table
            .select(users::id)
            .order(users::id.desc())
            .first(c)
    }
}