mod compiler{
    pub mod lexer;
    pub mod parser;
}

use compiler::lexer::*;
use compiler::parser::*;

fn main() {
    let select_query = "SELECT name, value FROM table WHERE value > 5 AND name == 'Jane Doe';";

    let select_query1 = "SELECT name, value FROM table WHERE (value >= 10 AND (name == 'Jane Doe' OR name == 'John Doe'));";

    let select_query2 = "SELECT name, value FROM table WHERE (((name == 'Alice' OR name == 'Bob') AND (value >= 7 OR value < 10)) OR (name == 'Charlie' AND (value <= 5 OR value > 12))) AND (value != 8 AND (name != 'Dave' OR value < 15));";
    let select_query3 = "SELECT name, value FROM table WHERE (value < 5 AND (name == 'Bob' OR name == 'Robert'));";

    let select_query4 = "SELECT name, value FROM table WHERE (name LIKE 'C%' AND (value > 7 OR value < 9));";

    let select_query5 = "SELECT name, value FROM table WHERE (value BETWEEN 10 AND 15 AND (name == 'Eve' OR name == 'Eva'));";




    let insert_query = "INSERT INTO CatsAndOwners(CatID, CatName, CatAge, CatColor, CatOwnerName, City)
VALUES
(2, 'Белла', 7, 'Белая', 'Максим', 'Саратов');";
    let queries = vec![select_query,
                       select_query1,
                       select_query2,
                       select_query3,
                       select_query4,
                       select_query5,

                       insert_query];

    for q in queries {
        let mut lexer = Lexer::new(q);
        let tokens = lexer.tokenize();
        println!("{:?}", tokens);
        let mut parser = Parser::new(&tokens);
        if let Some(query) = parser.parse() {
            println!("{:#?}", query);
        } else {
            println!("Failed to parse query.");
        }
    }


}
