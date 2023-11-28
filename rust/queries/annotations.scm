 (
   (comment) @annotation
   .
   (expression_statement
     (assignment_expression
       left: (identifier) @name))
  (#match? @annotation "///[ \t]*@class:[a-zA-Z0-9_]+")
 ) @all

(
  (comment) @annotation
  .
  (lexical_declaration
    (variable_declarator
      name: (identifier) @name))
  (#match? @annotation "///[ \t]*@class:[a-zA-Z0-9_]+")
) @all
