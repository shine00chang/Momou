; call expression of function
(
  (call_expression
    function: (identifier) @name) @invocation
  (#not-match? @name "^(require)$")
)

; call expression of member function
(call_expression 
  function: (member_expression
    property: (property_identifier) @name) @expr 
  arguments: (_)
) @invocation

; class instantiation
(new_expression
  constructor: (_) @name) @invocation
