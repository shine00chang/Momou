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

