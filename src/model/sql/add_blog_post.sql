INSERT INTO
  public.blog_posts (title, content, author)
VALUES
  ($1, $2, $3)
RETURNING id