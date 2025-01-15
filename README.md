### This currency is dedicated to reminiscence the figure of Amy

[//]: # (![equation]&#40;http://latex.codecogs.com/gif.latex?Concentration%3D%5Cfrac%7BTotalTemplate%7D%7BTotalVolume%7D&#41;  )

[//]: # ()
[//]: # ()
[//]: # ($$`\left&#40; \sum_{k=1}^n a_k b_k \right&#41;^2 \leq \left&#40; \sum_{k=1}^n a_k^2 \right&#41; \left&#40; \sum_{k=1}^n b_k^2 \right&#41;`$$)

[//]: # ()
[//]: # ()
[//]: # (```math)

[//]: # (\left&#40; \sum_{k=1}^n a_k b_k \right&#41;^2 \leq \left&#40; \sum_{k=1}^n a_k^2 \right&#41; \left&#40; \sum_{k=1}^n b_k^2 \right&#41;)

[//]: # (```)

###### Make sure for installing rust in ur system

### Project Tree
```textmate
NephophileCurrency
|- migrations
|  '- .keep
|- src
|  |- adapters
|  |  |- gateway
|  |  |  |- postgres
|  |  |  |  |- mod.rs
|  |  |  |  |- models.rs
|  |  |  |  '- schema.rs
|  |  |  '- mod.rs
|  |  '- mod.rs
|  |- application
|  |  |- mappers
|  |  |  '- mod.rs
|  |  |- repositories
|  |  |  |- mod.rs
|  |  |  '- repositories.rs 
|  |  |- usecases
|  |  |  |- mod.rs
|  |  |  '- interactor.rs  
|  |  |- utils
|  |  |  '- mod.rs
|  |  '- mod.rs
|  |- domain
|  |- infrastructure
|  |- main.rs
|  '- schema.rs
|- Cargo.lock
|- Cargo.toml
'- diesel.toml
```