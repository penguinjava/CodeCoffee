/*게시판 테이블*/
CREATE TABLE posts (
   id SERIAL PRIMARY KEY,
   title VARCHAR(255) NOT NULL,
   content TEXT NOT NULL,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

/*게시판 테이블*/
CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) UNIQUE NOT NULL
);


/*게시판과 태그를 연결*/
CREATE TABLE post_tags (
   post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
   tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
   PRIMARY KEY (post_id, tag_id)
);
