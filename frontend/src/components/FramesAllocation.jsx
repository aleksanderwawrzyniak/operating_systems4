import React from 'react';

import Header from "./Header";
import ProcessPopup from "./ProcessPopup";
import Body from './Body'
import OutputPopup from "./OutputPopup";

class FrameAllocation extends React.Component {
    state = {
        frameAllocationMethod: "equal",
        algorithm: "lru",
        programs: [],
        simulationResults: [],
        showProcessPopup: false,
        showOutputPopup: false,
        outputToShow: null,
        deletedProcesses: 0,
    };

    genPrograms = (data) => {
        let programs = [];

        for (let i = 0; i < data.programsNumber; i++) {
            let refsNumber = Math.floor((Math.random() * (data.maxRequests - data.minRequests)) + data.minRequests + 1);
            let refs = this.generateReferences(refsNumber);
            programs.push({
                id: i,
                size: refsNumber,
                requests: refs,
            });
        }

        console.log(programs);

        this.setState({
            programs: programs,
            deletedProcesses: 0
        });

        console.log(this.state.programs);
    };

    generateReferences = (n) => {
        let refs = "";
        for (let i = 0; i < n; i++) {
            refs += Math.floor(Math.random() * 10);
            refs += ' ';
        }
        return refs.trim();
    };

    selectAlgorithm = (value) => {
        this.setState({
            algorithm: value
        });
    };

    selectMethod = (value) => {
        this.setState({
            frameAllocationMethod: value
        });
    };

    showProcessPopup = () => {
        this.setState({showProcessPopup: true});
    };

    closeProcessPopup = () => {
        this.setState({showProcessPopup: false});
    };

    handleProcessChange = (id, value) => {
        this.setState({
            programs: this.state.programs.map(program => {
                if (program.id === id) {
                    let refs = value.trim().replace(/[^\s\d]/g, '');
                    return {
                        ...program,
                        size: (refs.length + 1) / 2,
                        requests: refs,
                    };
                } else {
                    return program;
                }
            })
        })
    };

    simulate = (info) => {
        if (this.state.programs.length === 0) {
            alert("Please generate programs first, to do that, click 'Randomize Programs' button!");
            return;
        }
        let request = 'http://localhost:8000/' + this.state.frameAllocationMethod;

        let json = {
            no_frames: Number(info.frames),
            no_processes: Number(info.processNumber) - this.state.deletedProcesses,
            algorithm: this.state.algorithm,
            interval: Number(info.interval),
            processes: this.state.programs
        };
        let as_string = JSON.stringify(json);

        fetch(request, {
            headers: {"Content-type": "text/plain; charset=UTF-8"},
            method: 'POST',
            body: as_string
        }).then(response => response.json()).then(data => {
            console.log(data);
            if (data.state !== "Good") {
                alert('Something went wrong with the simulation!');
            } else {
                data.method = this.state.frameAllocationMethod.toUpperCase();
                this.setState({
                    simulationResults: [...this.state.simulationResults, data]
                });
            }
        });
    };

    openResultPopup = (output) => {
        console.log('opening result popup');
        this.setState(
            {
                showOutputPopup: true,
                outputToShow: output
            });
    };

    closeResultPopup = () => {
        this.setState({
            openOutputPopup: false,
            outputToShow: null
        });
    };

    deleteSimulationResult = (index) => {
        this.setState({
            simulationResults: this.state.simulationResults.filter((ret, i) => i !== index)
        });
    };

    deletePrograms = (id) => {
        this.setState({
            programs: this.state.programs.filter(process => process.id !== id),
            deletedProcesses: this.state.deletedProcesses + 1
        });
    };


    render() {
        const showProcessPopup = this.state.showProcessPopup;
        const showOutputPopup = this.state.showOutputPopup;
        return (
            <div>
                {!showProcessPopup ?
                    (
                        <div>
                            {!showOutputPopup || this.state.outputToShow === null ?
                                (
                                    <div>
                                        <Header
                                            generatePrograms={this.genPrograms}
                                            setAlgorithm={this.selectAlgorithm}
                                            setMethod={this.selectMethod}
                                            startSimulation={this.simulate}
                                            openPopup={this.showProcessPopup}
                                        />
                                        <Body
                                            results={this.state.simulationResults}
                                            openPopup={this.openResultPopup}
                                            onDelete={this.deleteSimulationResult}
                                        />
                                    </div>
                                ) : (
                                    <OutputPopup data={this.state.outputToShow} closePopup={this.closeResultPopup}/>
                                )
                            }
                        </div>
                    ) : (
                        <ProcessPopup
                            closePopup={this.closeProcessPopup}
                            programs={this.state.programs}
                            onChange={this.handleProcessChange}
                            onDelete={this.deletePrograms}
                        />
                    )
                }
            </div>
        );
    };
}

export default FrameAllocation;
