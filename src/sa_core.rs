use rand::prelude::*;

#[derive(Default)]
pub struct SAGeneratorOptions {
    pub problems_num: i32,
    pub problems_ceiling: i32,
    pub problems_floor: i32,
    pub allow_negatives: bool,
    pub operation: SAOperation,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum SAOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct SAProblem {
    pub number: i32,
    pub constant1: i32,
    pub constant2: i32,
    pub operation: SAOperation,
    pub result: i32,
}

///Utilized for holding user-entered data and testing conversion to a number. Only once it's
///proven valid do we edit the internal data structure.
pub struct SAProblemInputBuffer {
    pub num_of_problems: String,
    pub floor: String,
    pub ceiling: String,
}

pub enum SAValidity {
    ValidNumber(i32),
    InvalidNumber,
}

impl SAGeneratorOptions {
    pub fn new() -> Self {
        SAGeneratorOptions {
            problems_num: 0,
            problems_ceiling: 0,
            problems_floor: 0,
            allow_negatives: false,
            operation: SAOperation::Addition,
        }
    }

    pub fn from(
        problems_num: i32,
        problems_ceiling: i32,
        problems_floor: i32,
        allow_negatives: bool,
        operation: SAOperation,
    ) -> SAGeneratorOptions {
        SAGeneratorOptions {
            problems_num,
            problems_ceiling,
            problems_floor,
            allow_negatives,
            operation,
        }
    }
}

impl SAOperation {
    pub fn new() -> Self {
        SAOperation::Addition
    }

    ///SAOperation::from() - converts String into SAOperation variant.
    ///
    ///Exhaustive, currently supports either the term itself (e.g. 'Addition') or the three-letter shorthand
    ///abbreviation for that term (e.g. 'add', 'sub', 'mul').
    pub fn from(input: String) -> SAOperation {
        match input.to_lowercase().as_str() {
            "add" | "addition" => SAOperation::Addition,
            "sub" | "subtraction" => SAOperation::Subtraction,
            "mul" | "multiplication" => SAOperation::Multiplication,
            "div" | "division" => SAOperation::Division,
            _ => {
                eprintln!("\nERR: Unhandled/Unsupported string entered for SAOperation::from!\nReturning default!...\n");
                SAOperation::Addition
            }
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            SAOperation::Addition => "Addition".to_owned(),
            SAOperation::Subtraction => "Subtraction".to_owned(),
            SAOperation::Multiplication => "Multiplication".to_owned(),
            SAOperation::Division => "Division".to_owned(),
        }
    }

    pub fn as_symbol(&self) -> char {
        match self {
            SAOperation::Addition => '+',
            SAOperation::Subtraction => '-',
            SAOperation::Multiplication => '*',
            SAOperation::Division => '/',
        }
    }
}

impl Default for SAOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl SAProblem {
    pub fn new() -> Self {
        SAProblem {
            number: 0,
            constant1: 0,
            constant2: 0,
            operation: SAOperation::new(),
            result: 0,
        }
    }

    pub fn student_string(&self) -> String {
        let operation_char = self.operation.as_string().chars().nth(0).unwrap();

        format!(
            "Problem {0}{1}: {2} {3} {4} = \n",
            operation_char,
            self.number,
            self.constant1,
            self.operation.as_symbol(),
            self.constant2
        )
    }

    pub fn instructor_string(&self) -> String {
        //Really, it's pretty stupid that for cases like this I seemingly
        //cannot just index the resultant string and instead have to query it
        //like some uncertain operation. This should only be logically required
        //if we're operating with one-way-blind strings or something.
        let operation_char = self.operation.as_string().chars().nth(0).unwrap();

        format!(
            "Problem {0}{1}: {2} {3} {4} = {5}\n",
            operation_char,
            self.number,
            self.constant1,
            self.operation.as_symbol(),
            self.constant2,
            self.result
        )
    }
}

impl Default for SAProblem {
    fn default() -> Self {
        Self::new()
    }
}

impl SAProblemInputBuffer {
    pub fn new() -> Self {
        SAProblemInputBuffer {
            num_of_problems: "".to_owned(),
            floor: "".to_owned(),
            ceiling: "".to_owned(),
        }
    }
}

impl Default for SAProblemInputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn sa_export(data: Vec<SAProblem>) -> anyhow::Result<(String, String)> {
    use rand::distributions::{Alphanumeric, DistString};
    use std::{
        fs::File,
        io::{prelude::*, LineWriter},
    };

    let tag = {
        let operation_buffer = data[0].operation;
        let mut string_buffer = data[0].operation.as_string();

        for problem in &data {
            if operation_buffer != problem.operation {
                string_buffer = "Mixed".to_owned();
                break;
            } else {
                continue;
            }
        }

        string_buffer
    };
    let keycode = Alphanumeric.sample_string(&mut rand::thread_rng(), 5);
    let timestamp = chrono::offset::Local::now().date_naive().format("%m-%d-%Y");
    let student_filename = format!("SimAri_Student_{timestamp}_{tag}_{keycode}.txt");
    let instructor_filename = format!("SimAri_Instructor_{timestamp}_{tag}_{keycode}.txt");
    let mut student_file_out = LineWriter::new(File::create(&student_filename)?);
    let mut instructor_file_out = LineWriter::new(File::create(&instructor_filename)?);

    for problem in data {
        student_file_out.write_all(problem.student_string().as_bytes())?;
        instructor_file_out.write_all(problem.instructor_string().as_bytes())?;
    }

    Ok((student_filename, instructor_filename))
}

pub fn sa_generate(options: &SAGeneratorOptions) -> Vec<SAProblem> {
    use rand::distributions::Uniform;

    let mut generated_package = Vec::new();
    let mut used_number_buffer = Vec::new();
    let mut used_answer_buffer = Vec::new();
    let mut last_constant1 = 0i32;
    let mut last_constant2 = 0i32;
    let perc = ((options.problems_ceiling - options.problems_floor) / 100) * 12;

    while generated_package.len() < options.problems_num as usize {
        let mut rng1 = thread_rng();
        let mut rng2 = thread_rng();
        let constant1 =
            Uniform::from(options.problems_floor..=options.problems_ceiling).sample(&mut rng1);
        let constant2 =
            Uniform::from(options.problems_floor..=options.problems_ceiling).sample(&mut rng2);
        let problem: SAProblem = SAProblem {
            number: generated_package.len() as i32 + 1,
            constant1,
            constant2,
            operation: options.operation,
            result: match options.operation {
                SAOperation::Addition => constant1 + constant2,
                SAOperation::Subtraction => constant1 - constant2,
                SAOperation::Multiplication => constant1 * constant2,
                SAOperation::Division => constant1 / constant2,
            },
        };

        if (problem.constant1 - perc..=problem.constant1 + perc).contains(&problem.constant2)
            || (problem.result <= 0 && !options.allow_negatives)
        {
            continue;
        }

        if generated_package.len() > 1 {
            if (problem.constant1 - perc..=problem.constant1 + perc).contains(&last_constant1)
                || (problem.constant2 - perc..=problem.constant2 + perc).contains(&last_constant2)
                || used_number_buffer.contains(&problem.constant1)
                || used_number_buffer.contains(&problem.constant2)
                || used_answer_buffer.contains(&problem.result)
            {
                continue;
            }
        }

        used_number_buffer.push(problem.constant1);
        used_number_buffer.push(problem.constant2);
        used_answer_buffer.push(problem.result);
        last_constant1 = problem.constant1;
        last_constant2 = problem.constant2;

        generated_package.push(problem);
    }

    generated_package
}
