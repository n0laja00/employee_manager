pub struct NoMode;
pub struct EmployeeMode;
pub struct WorkHourMode;
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Command {
    Add,
    Remove,
    Edit,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Employee {
    pub department: String,
    pub first_name: String,
    pub last_name: String,
    pub id: u64,
}
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct EmployeeModeInput {
    pub command: Command,
    pub employee: Employee,
}
#[derive(Debug)]
pub struct WorkHourModeInput {
    pub command: Command,
    pub workhour: WorkHour,
}
#[derive(Debug)]
pub struct Input<State = NoMode> {
    pub command: Command,
    pub body: String,
    pub state: std::marker::PhantomData<State>,
}

#[derive(Debug, PartialEq)]
pub struct WorkHour {
    pub hours: f64,
    pub id: u64,
    pub employee_id: u64,
}
impl WorkHour {
    pub fn new(hours: f64, id: u64, employee_id: u64) -> Self {
        WorkHour {
            hours,
            id,
            employee_id,
        }
    }
    pub fn add_hours(&self, hours: f64) -> f64 {
        self.hours + hours
    }
    pub fn remove_hours(&self, hours: f64) -> f64 {
        self.hours - hours
    }
    pub fn get_hours(&self) -> f64 {
        self.hours
    }
}

impl Input<NoMode> {
    pub fn employee_mode(command: Command, body: String) -> Input<EmployeeMode> {
        Input {
            command: command,
            body: body,
            state: std::marker::PhantomData::<EmployeeMode>,
        }
    }
    pub fn workhour_mode(command: Command, body: String) -> Input<WorkHourMode> {
        Input {
            command: command,
            body: body,
            state: std::marker::PhantomData::<WorkHourMode>,
        }
    }
}

impl Input<WorkHour> {
    pub fn make_workhour_input(command: Command, workhour: WorkHour) -> WorkHourModeInput {
        WorkHourModeInput { command, workhour }
    }
}
impl Input<EmployeeMode> {
    pub fn make_employee_input(command: Command, employee: Employee) -> EmployeeModeInput {
        EmployeeModeInput { command, employee }
    }
}

impl Employee {
    pub fn new(department: String, first_name: String, last_name: String, id: u64) -> Self {
        Employee {
            department,
            first_name,
            last_name,
            id,
        }
    }
}
