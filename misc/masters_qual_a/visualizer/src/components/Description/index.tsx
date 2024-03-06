import { CONFIG } from 'config';
import type { FC } from 'react';

const Description: FC = () => (
  <div>
    <div>
      <h3>{CONFIG.title}</h3>
      <p>{CONFIG.description}</p>
      {CONFIG.link && (
        <a href={CONFIG.link.href} target="_blank" rel="noopener noreferrer">
          {CONFIG.link.text}
        </a>
      )}
    </div>
  </div>
);

export default Description;
