import React from 'react';
import '../styles/OutputProcess.css';

class OutputProcess extends React.Component {

    render() {
        const {process} = this.props;
        return (
            <div className="process">
                <div>
                    <span className="span id-div">id: {process.id}</span>
                    <span className="span size-div">size: {process.size}</span>
                </div>
                <div className="fp-div">fault percentage: {Math.round(process.fault_percentage * 100) / 100}</div>
                <div className="frames-div">number of frames: {process.no_frames}</div>
            </div>
        )
    }
}

export default OutputProcess;