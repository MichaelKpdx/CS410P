Rust Web Example
Michael Kemp
This repository will be used to save the changes I make to the homework assignments throughout the class. All code will be committed from vs code to this repository.

In order to run the app you need to go into the working directory and run docker-compose up. After that in order to enter data into the postgres tables you need to run docker ps. This will give you the list of the active working containers. Copy the postgres container ID. Next run run docker exec -it (postgres container ID) bash. Then enter the command psql and copy the following command. This will give data in the tables to test the app with.

INSERT INTO questions(id,title,content)
VALUES('color','color','Whats your favorite color'),
('first','first question','what is this class'),
('second','second question','what is your name');
