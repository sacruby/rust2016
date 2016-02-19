// This app is a mock command line interface for listing and filtering products
// by labels.  The Structs are created for you and the tests are in place to
// check if you are getting the desired values from the CLI program.  First step
// is to get the program to compile. Hint: look at the run_cli() function at the
// bottom of the file.  Remember the `-> String means that this function should
// return a String.  Once you figure out how to make the program compile try getting
// one test to pass at a time.  The simplest test to get to pass is the
// `run_program_error_test` so that is a good place to start.  The tests are in
// order of difficulty below so once you have the error test done move on to the
// product list.  Finally see if you can get the filter test to pass by completing
// the Product::filter() function.

fn main() {
    run_program("ls products".to_string());
}

//tests *******************************************************
#[test]
fn run_program_error_test() {
    assert_eq!(
        "Invalid command".to_string(),
        run_program("not a command".to_string())
    );
}

#[test]
fn run_program_products_test() {
    assert_eq!(
        "[Product { id: 1, name: \"Awesome Gaget\", label: \"awesome\" }, \
          Product { id: 2, name: \"Cool Widget\", label: \"cool\" }, \
          Product { id: 3, name: \"Rad Gizmo\", label: \"cool\" }, \
          Product { id: 4, name: \"Lame Thingy\", label: \"lame\" }\
        ]".to_string(),
        run_program("ls products".to_string())
    );
}

#[test]
fn run_program_labels_test() {
    assert_eq!(
        "[Label { id: 1, text: \"cool\" }, \
          Label { id: 2, text: \"awesome\" }, \
          Label { id: 3, text: \"lame\" }\
        ]".to_string(),
        run_program("ls labels".to_string())
    );
}

#[test]
fn run_program_filter_by_test() {
    assert_eq!(
        "Product { id: 1, name: \"Awesome Gaget\", label: \"awesome\" }".to_string(),
        run_program("filter by awesome".to_string())
    );
}

// Models ***************************************************

#[derive(Debug)]
struct Label {
    id: i32,
    text: &'static str,
}

#[derive(Debug)]
struct Product {
    id: i32,
    name: &'static str,
    label: &'static str,
}

impl Product {
    fn filter(l: String, products: Vec<Self>) -> String {
        let label = l.split(' ').collect::<Vec<&str>>()[2];
        let mut products_string = String::new();
        for product in products {
          if product.label == label {
            products_string = products_string + &format!("{:?}", product);
          }
        }
        products_string
    }
}


// Program setup *********************************************
fn run_program(command: String) -> String {
    let p1 = Product {id: 1, name: "Awesome Gaget", label: "awesome"};
    let p2 = Product {id: 2, name: "Cool Widget", label: "cool"};
    let p3 = Product {id: 3, name: "Rad Gizmo", label: "cool"};
    let p4 = Product {id: 4, name: "Lame Thingy", label: "lame"};
    let products = vec![p1, p2, p3, p4];

    let l1 = Label {id: 1, text: "cool"};
    let l2 = Label {id: 2, text: "awesome"};
    let l3 = Label {id: 3, text: "lame"};
    let labels = vec![l1, l2, l3];

    run_cli(products, labels, command)
}


// mocking a CLI interface **********************************
// command is the command that is passed in by the user
// ls command is for listing a resource.  i.e. `ls products will list all
// of the products.  `filter by <label>` should use the product filter function
// to filter products by the given label.

fn run_cli(products: Vec<Product>, labels: Vec<Label>, command: String) -> String {
    let mut std_out = "Invalid command".to_string();
    if command == "ls products" {
      std_out = format!("{:?}", products)
    } else if command== "ls labels" {
      std_out = format!("{:?}", labels);
    } else if command.starts_with("filter by") {
      std_out = Product::filter(command, products)
    }
    println!("{}", std_out);
    std_out
}
