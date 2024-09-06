UPDATE blog_posts
SET
  content = $3,
  title = $2
WHERE
  id = $1
  AND author = $4