import React, {useState} from 'react';
import {useStreamControl} from './useStreamControl';
import {useNear} from '../near-connect/useNear';
import {Button, Input} from '../../components/kit';
import {StreamAutodepositStatus} from './StreamAutodepositStatus';
import {STREAM_STATUS} from './lib';
import classNames from 'classnames';


export function StreamAutodepositButton({stream, className, disableMsg, enableMsg}) {
  const controls = useStreamControl(stream.stream_id);
  return (
    <>
      <Button
        variant="filled"
        type="button"
        className={className}
        enabled
        onClick={() =>
          stream.auto_deposit_enabled ? controls.disable() : controls.enable()
        }
      >
        {stream.auto_deposit_enabled ? disableMsg || 'Disable': enableMsg || 'Enable'}
      </Button>
    </>
  );
}


export function StreamAutodepositControls({stream, minimal, className, disableMsg, enableMsg, buttonClassName}) {
  const near = useNear();
  const isDead =
    stream.status === STREAM_STATUS.INTERRUPTED ||
    stream.status === STREAM_STATUS.FINISHED;
  const controls = useStreamControl(stream.stream_id);

  if (controls.loading) {
    return <span>Loading!</span>;
  }

  if (isDead) {
    return <StreamAutodepositStatus stream={stream} enableMsg={enableMsg} disableMsg={disableMsg}/>;
  }

  return (
    <div className={classNames(className, 'twind-relative twind-inline-flex')}>
        <StreamAutodepositButton
          className={classNames(buttonClassName)}
          stream={stream}
          enableMsg={enableMsg}
          disableMsg={disableMsg}
        />
    </div>
  );
}