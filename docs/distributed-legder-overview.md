# Distributed Legder



# diagram

```plantuml
@startuml
  class Node {
    - Key id
    - String ip
    - int port 
    
    + getIntfo()
    + getAdrr()
    * getKey()
  } 
  
  class Key {
    - Key id
    - String name
    - int number 
    
    +void getName()
    +void getNumber()
    +String toString()
  }
  
  enum Message {
    - PING
    - STORE
    - FIND_NODE
    - FIND_VALUE
  }
  
  class Server {
    - Request=Call
    - Response
  }
  
  class Client { 
    - Request
    - Response
  }
  
  
   
@enduml
```

