use std::panic::Location;

enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}


#[derive(Debug)]
struct Database;

#[derive(Clone, Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

impl Database {
    fn connect() -> Result<Self, String> {
        // In a production application, a database connection error is likely to occur here.
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee {
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee {
                name: "Brody".to_string(),
            }),
            "Catherine" => Ok(Employee {
                name: "Catherine".to_string(),
            }),
            _ => Err(String::from("employee not found")),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
        match employee.name.as_str() {
            "Anita" => Ok(KeyCard { access_level: 1000 }),
            "Brody" => Ok(KeyCard { access_level: 500 }),
            other => Err(format!("{other} doesn't have a keycard")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {

    let db = Database::connect()?;
    let employee = Database::find_employee(&db, &employee_name)?;
    let emp_keycard = Database::get_keycard(&db, &employee)?;

    let location_access_level = ProtectedLocation::required_access_level(&location);

    if emp_keycard.access_level >= location_access_level{
        Ok(AuthorizationStatus::Allow)
    }else {
        Ok(AuthorizationStatus::Deny)
    }
}


fn main(){
    // Anita is trying to access the Warehouse, which requires access level 500.
    // Her keycard has access level 1000, which should be allowed.
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);
    // Brody is trying to access the Office, which requires access level 800.
    // His keycard has access level 500, which should be denied.
    let brody_authorized = authorize("Brody", ProtectedLocation::Office);
    // Catherine is trying to access the Warehouse, which requires access level 500.
    // She doesn't have a keycard, so this should be an error.
    let catherine_authorized = authorize("Catherine", ProtectedLocation::Warehouse);

    println!("{anita_authorized:?}");
    println!("{brody_authorized:?}");
    println!("{catherine_authorized:?}");
}