import React from 'react';
import * as icons from './icons';
import {NavLink} from './kit';
import {NearAuthButton} from '../features/near-auth/NearAuthButton';
import classNames from 'classnames';
import {routes} from '../lib/routing';
import LogoText from '../images/logo_stream_with_text.svg';

export function Header() {
  const navigation = (
    <ul className="twind-flex-col lg:twind-flex-row twind-flex twind-justify-center ">
      <li className="twind-mb-2 lg:twind-mr-2 lg:twind-mb-0">
        <NavLink to={routes.myStreams} icon={<icons.Streams />}>
          My Streams
        </NavLink>
      </li>
      <li className="twind-mb-2 lg:twind-mr-2 lg:twind-mb-0">
        <NavLink to={routes.send} icon={<icons.Send />}>
          Send
        </NavLink>
      </li>
    </ul>
  );

  const logo = (
    <div className="twind-flex twind-justify-start twind-items-center">
      <img src={LogoText} alt="xyiming logo" />
    </div>
  );
  return (
    <div className="twind-py-4 twind-px-6">
      <div
        className={classNames(
          'twind-hidden lg:twind-grid twind-items-center twind-grid-cols-3 twind-gap-3 ',
        )}
      >
        {logo}
        {navigation}
        <div className="twind-flex twind-justify-end">
          <NearAuthButton />
        </div>
      </div>

      <div className={classNames('lg:twind-hidden', 'navbar-dark')}>
        <div className="twind-flex twind-justify-between">
          {logo}

          <button
            className="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarSupportedContent"
            aria-controls="navbarSupportedContent"
            aria-expanded="false"
            aria-label="Toggle navigation"
          >
            <span className="navbar-toggler-icon" />
          </button>
        </div>
        <div
          className="collapse navbar-collapse twind-mt-4"
          id="navbarSupportedContent"
        >
          {navigation}
          <NearAuthButton className="mt-4" />
        </div>
      </div>
    </div>
  );
}
