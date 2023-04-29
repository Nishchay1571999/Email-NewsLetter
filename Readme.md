# About :

## The project will fullfill these three needs:
    - As a blog visitor,
        I want to subscribe to the newsletter,
        So that I can receive email updates when new content is published on the blog;
    - As the blog author,
        I want to send an email to all my subscribers,
        So that I can notify them when new content is published;
    - As a subscriber,
        I want to be able to unsubscribe from the newsletter,
        So that I can stop receiving email updates from the blog.

## Strategy 
1. Choosing a Web framework
2. Define a Testing Strategy
3. Choose a create to save the email list in the database
4. Define how to make changes to the database from time to time
5. Write some queries to the database


## Choosing a Web Framework
    - actix-web:    
        does not have an configuration management system ORM integration out of the box,Could easily add Prometheus metrics, Easier to deploy distributed tracing(tracking or monitoring the requests in applications built with microservices(services which are used to do a specific task individually) )
    - tide:
        does not have an configuration management system ORM integration out of the box, 
    - wrap:
        does not have an configuration management system ORM integration out of the box,
    - rocket:
        has a set of high quality tree structure to build everyday applications, Could easily add Prometheus metrics