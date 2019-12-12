import React from 'react';
import Steps from './Steps';
import '../styles/Programme.css';

class Programme extends React.Component {
    render() {
        const {id, size, faults, fault_percentage, no_frames, steps} = this.props.self;
        console.log(steps);
        return (
            <div className="programme">
                <div className="container">id: {id}</div>
                <div className="container">size: {size}</div>
                <div className="container">faults: {faults}</div>
                <div className="container">miss rate: {Math.round(fault_percentage * 100)}%</div>
                <div className="container">frames: {no_frames}</div>
                <Steps steps={steps} />
            </div>
        )
    }
}

export default Programme;