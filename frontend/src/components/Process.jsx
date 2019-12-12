import React from 'react';
import '../styles/Process.css';

class Process extends React.Component {
    render() {
        return (
            <div className="my-process">
                <button className="del-btn" onClick={() => this.props.onDelete(this.props.id)}>delete</button>
                <div className="span-field">
                    <span className="my-span">id: {this.props.id}</span>
                    <span className="my-span">size: {this.props.size}</span>
                </div>
                <div>
                    <textarea className="refs" value={this.props.requests}
                              onChange={event => this.props.onChange(this.props.id, event.target.value)}/>
                </div>
            </div>
        )
    }
}

export default Process;