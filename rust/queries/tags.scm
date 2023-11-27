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

; class definition
(
  (comment)* @doc
  .
  [
    (class
      name: (_) @name)
    (class_declaration
      name: (_) @name)
  ] @class
  (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
  (#select-adjacent! @doc @class)
)

; function definition
;; (
;;   (comment)* @doc
;;   .
;;   [
;;     (function
;;       name: (identifier) @name)
;;     (function_declaration
;;       name: (identifier) @name)
;;     (generator_function
;;       name: (identifier) @name)
;;     (generator_function_declaration
;;       name: (identifier) @name)
;;   ] @definition.function
;;   (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
;;   (#select-adjacent! @doc @definition.function)
;; )

;; ; something about arrow functions
;; (
;;   (comment)* @doc
;;   .
;;   (lexical_declaration
;;     (variable_declarator
;;       name: (identifier) @name
;;       value: [(arrow_function) (function)]) @definition.function)
;;   (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
;;   (#select-adjacent! @doc @definition.function)
;; )
;;
;; ; named arrow function declaration
;; (
;;   (comment)* @doc
;;   .
;;   (variable_declaration
;;     (variable_declarator
;;       name: (identifier) @name
;;       value: [(arrow_function) (function)]) @definition.function)
;;   (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
;;   (#select-adjacent! @doc @definition.function)
;; )
;;
;; ; named arrow function assignment 
;; (assignment_expression
;;   left: [
;;     (identifier) @name
;;     (member_expression
;;       property: (property_identifier) @name)
;;   ]
;;   right: [(arrow_function) (function)]
;; ) @definition.function
;;
;; ; arrow function declaration within object
;; (pair
;;   key: (property_identifier) @name
;;   value: [(arrow_function) (function)]) @definition.function
;;
; call expression
(
  (call_expression
    function: (identifier) @name) @reference.call
  (#not-match? @name "^(require)$")
)

; call expression
(call_expression 
  function: (member_expression) @name
  arguments: (_) @reference.call
) 

; class instantiation
(new_expression
  constructor: (_) @name) @definition.instantiation 

; class annotation

