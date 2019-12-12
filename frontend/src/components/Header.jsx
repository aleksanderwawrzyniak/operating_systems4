import React from "react";
import SettingInputs from "./SettingInputs";
import SettingButtons from './SettingButtons';

import "../styles/Header.css";
import StartingSection from "./StartingSection";

class Header extends React.Component {
    state = {
        minRequests: 50,
        maxRequests: 150,
        framesNumber: 35,
        programsNumber: 15,
        intervalLength: 6,
    };

    handleMinRequestsChange = (value) => {
        if (value > 0) {
            this.setState({
                minRequests: value
            })
        } else {
            this.setState({
                minRequests: 1
            })
        }
        console.log(this.state.minRequests);
    };

    handleMaxRequestsChange = (value) => {
        if (value > this.state.minRequests) {
            this.setState({
                maxRequests: value
            })
        } else {
            this.setState({
                maxRequests: this.state.minRequests + 1
            })
        }
    };

    handleFramesNumberChange = (value) => {
        if (value > 0) {
            this.setState({
                framesNumber: value
            })
        } else {
            this.setState({
                framesNumber: 1
            })
        }
    };

    handleProgramsNumberChange = (value) => {
        if (value > 0) {
            this.setState({
                programsNumber: value
            })
        } else {
            this.setState({
                programsNumber: 1
            })
        }
    };

    handleIntervalLengthChange = (value) => {
        if (value > 0) {
            this.setState({
                intervalLength: value
            })
        } else {
            this.setState({
                intervalLength: 1
            })
        }
    };

    randomizePrograms = () => {
        console.log(this.props);
        const {generatePrograms} = this.props;
        console.log('called randomizePrograms');
        generatePrograms({
            programsNumber: this.state.programsNumber,
            minRequests: this.state.minRequests,
            maxRequests: this.state.maxRequests
        });
    };

    simulate = () => {
        console.log('start simulation');
        const { startSimulation } = this.props;
        startSimulation({
            frames: this.state.framesNumber,
            processNumber: this.state.programsNumber,
            interval: this.state.intervalLength
        });
    };

    selectAlgorithm = (value) => {
        console.log('selectAlgorithm');
        const { setAlgorithm } = this.props;
        setAlgorithm(value);
    };

    selectMethod = (value) => {
        console.log('selectMethod');
        const { setMethod } = this.props;
        setMethod(value);
    };

    showPopup = () => {
        const { openPopup } = this.props;
        openPopup();
    };

    render() {
        return (
            <div className="my-header">
                <h4>Settings:</h4>
                <SettingInputs
                    handleMinRequestsChange={this.handleMinRequestsChange}
                    handleMaxRequestsChange={this.handleMaxRequestsChange}
                    handleFramesNumberChange={this.handleFramesNumberChange}
                    handleProgramsNumberChange={this.handleProgramsNumberChange}
                    handleIntervalLengthChange={this.handleIntervalLengthChange}
                    minRequests={this.state.minRequests}
                    maxRequests={this.state.maxRequests}
                    framesNumber={this.state.framesNumber}
                    programsNumber={this.state.programsNumber}
                    intervalLength={this.state.intervalLength}
                />
                <SettingButtons
                    minRequests={this.state.minRequests}
                    maxRequests={this.state.maxRequests}
                    framesNumber={this.state.framesNumber}
                    programsNumber={this.state.programsNumber}
                    randomizePrograms={this.randomizePrograms}
                    showPopup={this.showPopup}
                />
                <StartingSection
                    setAlgorithm={this.selectAlgorithm}
                    setMethod={this.selectMethod}
                    startSimulation={this.simulate}
                />
            </div>
        );
    }
}

export default Header;
