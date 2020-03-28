import * as React from "react";

export interface HelloWorldProps { firstName: string; lastName: string; }

// 'HelloWorldProps' describes our props structure.
// For the state, we use the '{}' type.
export class HelloWorld extends React.Component<HelloWorldProps, {}> {
    render() {
        return <h1>Hello World!</h1>;
    }
}