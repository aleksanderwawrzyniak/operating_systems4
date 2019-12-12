import React from "react";
import "../styles/SettingInputs.css";

class SettingInputs extends React.Component {

    showProps = () => {
        console.log(this.props);
    }

    render() {
        return (
            <div className="settings">
                <div className="input-holder input-holder-long">
                    <div>Min requests per process</div>
                    <input className="inp" value={this.props.minRequests} type="number"
                           onChange={event => this.props.handleMinRequestsChange(event.target.value)}/>
                </div>
                <div className="input-holder input-holder-long">
                    <div>Max requests per process</div>
                    <input className="inp" value={this.props.maxRequests} type="number"
                           onChange={event => this.props.handleMaxRequestsChange(event.target.value)}/>
                </div>
                <div className="input-holder">
                    <div>Number of frames</div>
                    <input className="inp" value={this.props.framesNumber} type="number"
                           onChange={event => this.props.handleFramesNumberChange(event.target.value)}/>
                </div>
                <div className="input-holder">
                    <div>Number of programs</div>
                    <input className="inp" value={this.props.programsNumber} type="number"
                           onChange={event => this.props.handleProgramsNumberChange(event.target.value)}/>
                </div>
                <div className="input-holder">
                    <div> interval length</div>
                    <input className="inp" value={this.props.intervalLength} type="number"
                           onChange={event => this.props.handleIntervalLengthChange(event.target.value)}/>
                </div>
            </div>
        )
    }
}

export default SettingInputs;
