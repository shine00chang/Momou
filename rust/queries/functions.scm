(function
  name: (identifier) @name
  ) @function

(function_declaration
  name: (identifier) @name
  ) @function

(method_definition
  name: (property_identifier) @name
  ) @function

(pair
  key: (property_identifier) @name
  value: [(function) (arrow_function)]) @function

(assignment_expression
  left: (member_expression
    property: (property_identifier)
    ) @name
  right: [(function) (arrow_function)]
  ) @function

(variable_declarator
  name: (identifier) @name 
  value: [(function) (arrow_function)]
  ) @function

(assignment_expression
  left: (identifier) @name
  right: [(function) (arrow_function)]
  ) @function
