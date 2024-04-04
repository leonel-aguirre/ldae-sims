ALTER TABLE
  courses
ADD
  COLUMN specialization_code VARCHAR(20);

ALTER TABLE
  courses
ADD
  CONSTRAINT courses_specialization_code_fkey FOREIGN KEY (specialization_code) REFERENCES specializations(specialization_code);