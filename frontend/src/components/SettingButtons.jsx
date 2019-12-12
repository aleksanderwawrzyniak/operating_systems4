import React from 'react';
import '../styles/SettingButtons.css';

class SettingButtons extends React.Component {

    render() {
        return (
            <div className="buttons">
                <div className="button-holder">
                    <button className="btn" type="button"
                            onClick={this.props.randomizePrograms}>Randomize Programs
                    </button>
                </div>
                <div className="button-holder">
                    <button className="btn" type="button" onClick={this.props.showPopup}>Extended Programs
                        Settings
                    </button>
                </div>
            </div>
        )
    }
}

export default SettingButtons;