import React, { Component, PropTypes } from 'react';

import Form, { FormWrap, Input } from '../../../Form';
import IdentityIcon from '../../../IdentityIcon';

import styles from '../style.css';

export default class RecoverAccount extends Component {
  static propTypes = {
    address: PropTypes.string,
    name: PropTypes.string,
    phrase: PropTypes.string
  }

  render () {
    let info = 'The details for your newly created account is displayed below. ';
    if (this.props.phrase) {
      info += 'Take note of your recovery phrase and store it in a secure location, without it you cannot recover your account should you lose your password.';
    }

    return (
      <Form>
        <IdentityIcon
          address={ this.props.address } />
        <div className={ styles.info }>
           { info }
        </div>
        <FormWrap>
          <Input
            disabled
            hint='a descriptive name for the account'
            label='account name'
            value={ this.props.name } />
        </FormWrap>
        <FormWrap>
          <Input
            disabled
            hint='the network address for the account'
            label='address'
            value={ this.props.address } />
        </FormWrap>
        { this.renderPhrase() }
      </Form>
    );
  }

  renderPhrase () {
    if (!this.props.phrase) {
      return null;
    }

    return (
      <FormWrap>
        <Input
          disabled
          hint='The account recovery phrase'
          label='Recovery Phrase'
          multiLine
          rows={ 1 }
          value={ this.props.phrase } />
      </FormWrap>
    );
  }
}