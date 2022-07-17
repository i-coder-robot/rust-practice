use crate::db;
use crate::error_handler::CustomError;
use crate::schema::employees;
use disel::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,AsChangeset,Insertable)]
#[table_name="employees"]
pub struct Employee{
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary:i32,
    pub age:i32,
}

#[derive(Serialize,Deserialize,AsChangeset,Insertable)]
pub struct Employees{
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary:i32,
    pub age:i32,
}

impl Employees{
    pub fn find_all()->Rsult<Vec<Self>,CustomError>{
        let conn=db::connection()?;
        let employees = employees::table.load::<Employees>(&conn)?;
        Ok(employees)
    }

    pub fn find(id:i32)->Result<Self,CustomError>{
        let conn=db::connection()?;
        let employee = employees::table.filter(employees::id.eq(id)).first(&conn)?;
        Ok(employee)
    }

    pub fn create(employee:Employee)->Result<Self,CustomError>{
        let conn=db::connection()?;
        let employee = Employee::from(employee);
        let employee = diesel::insert_into(employee::table)
            .values(employee).get_result(&conn)?;
        Ok(employee)
    }

    pub fn update(id:i32,employee:Employee)->Result<Self,CustomError>{
        let conn=db::connection()?;
        let employee = diesel::update(employee::table)
            .filter(employees::id.eq(id)).set(employee).get_result(&conn)?;
        Ok(employee)
    }

    pub fn delete(id:i32)->Result<usize,CustomError>{
        let conn=db::connection()?;
        let res = diesel::delete(employee::table.filter(employees::id.eq(id)))
            .execute(&conn)?;
        Ok(res)
    }
}

impl Employee{
    fn from(employee:Employee)->Employee{
        Employee{
            first_name: employee.first_name,
            last_name: employee.last_name,
            department: employee.department,
            salary : employee.salary,
            age:employee.age,
        }
    }
}
