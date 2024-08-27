INSERT INTO
  users (email, password_hash, display_name)
values
  ($1, $2, $3) 
RETURNING id