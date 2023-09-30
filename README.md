# Apirena: Modular OpenAPI Superset

Welcome to **Apirena**! This project is a modular superset of the OpenAPI specification focusing on enhancing the API playground experience with intuitive, developer-centric tools and features. 

We take the enhanced functionality from industry leaders like Postman, Insomnia & Hopscotch, and the run an API client in the browser based where you write your documentaiton in code using a fork of your favourite OpenAPI generators.

## Overview

API clients like Postman, Insomnia, and Hopscotch are awesome, but the API definitions & documentation live far away from your code and there it's easy to get out of sync from your code. 

Swagger-UI is cumbersome, on large projects with 100s of routes the page is glitchy and you don't get the awesome features like easily switching environments with environment variables. Swagger-UI is also limiting when you want to quickly test adding params that aren't in your documentation, or more dynamic param keys like search filters. 

![MacBook Pro 14_ - 1](https://github.com/apirena/apirena/assets/23046374/736622da-fdad-45b6-b18d-6c6f794318e9)


## Core Features

### 1. **Environment Variables**
- Define multiple environments like Production, Staging, and Development.
- Switch between environments effortlessly, updating API endpoints, headers, and environment-specific values on-the-fly.

### 2. **Enhanced Authentication**
- Wide-ranging support: Basic Auth, Bearer Token, OAuth1, OAuth2, and more.
- Auto-refresh tokens ensuring a continuous, unhindered testing experience.
- Auth drives from environment variables so you can switch between environments on the fly.

### 3. **Parameter Management**
- Intuitive UI for entering API parameters: easily toggle between form-data, raw JSON, multipart, and more.
- Smart auto-suggestions based on history and integrated docs.

### 4. **Testing & Supplemental Parameters**
- Rapidly toggle optional parameters, streamlining different scenario tests.
- Integrate response assertions and scripting for automated test workflows.
- Easily add parameters that aren't doccumented for rapid development testing.

### 5. **In-Code Documentation**  
- Write and update documentation directly within the codebase, ensuring it evolves with the code.
- In-built parsing to present the latest documentation in the UI during testing.

### 6. **API Playground UI**
- An immersive environment for developers to experiment with the API.
- Quick, responsive, and user-friendly, drawing inspiration from top tools but tailored to our objectives.

### 7. **OpenAPI 3 Superset**
- Start with OpenAPI 3.

### 8. **Focused Approach**
- While tightly integrated with documentation, Apirena is primarily a hands on API testing tool.
- Integrated documentation is a secondary advantage, ensuring developers have updated info at their fingertips.

## Additional Advantages

- **Performance**: Enhanced responsiveness with faster load times.
- **Maintainability**: Modularization promotes easier navigation and updates.
- **Collaboration**: Modular design allows multiple developers to collaborate without conflicts.

## Considerations

- The enhanced structure may introduce complexity in file management and reference maintenance.

## Implementation Roadmap

1. **Initial Setup**: Lay out the directory structure and sample modularized specs.
2. **Feature Development**: Incorporate Apirena's distinct functionalities.
3. **Tooling**: Build the resolver, live reload server, and other vital components.
4. **Feedback & Testing**: Collaborate with developers, refine based on their input, and undergo rigorous performance evaluations.

## Technical Stack

- Framework Packages to generate files from a command. @nestjs/swagger, laravel-openapi etc.
- Bun server that watches for file changes to OpenApi file
- Bun app writes data to a PPocketbase database (SQLite)
- SvelteKit app for frontend connected to PocketBase app
- Docker configuration would define the location to the OpenAPI file (& supplemental files)
- Bun app watches DB for changes and can write back to files that aren't code generated (environments/markdown docs)

## Conclusion

Apirena reimagines the API documentation and testing landscape. By prioritizing modularization and cutting-edge functionalities, API development and testing have never been this streamlined and intuitive.

## Contributing

We encourage open-source contributions! Please refer to the `CONTRIBUTING.md` for guidelines.

---

*For an in-depth understanding, consult the project's detailed RFC.*
