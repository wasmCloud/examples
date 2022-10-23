import React, { Component } from 'react';

export default class ErrorBoundary extends Component {
  constructor(props) {
    super(props);
    this.state = {
      hasError: false,
    }
  }

  static getDerivedStateFromError(error) {
    return { hasError: true };
  }

  componentDidCatch(error, info) {
    this.setState({
      hasError: error,
    })
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="text-white px-6 py-4 border-0 rounded-md relative mb-4 bg-red-500">
          <span className="inline-block align-middle mr-8">
            <b className="capitalize">Error&nbsp;</b>{this.state.hasError.message}
          </span>
          <button onClick={() => {
            this.setState({
              hasError: false,
            })
          }} className="absolute bg-transparent text-2xl font-semibold leading-none right-0 top-0 mt-4 mr-6 outline-none focus:outline-none">
            <span>×</span>
          </button>
        </div>
      )
    } else {
      return this.props.children;
    }
  }
}