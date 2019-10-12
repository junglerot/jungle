import React from 'react';
import styled, {css} from 'styled-components';
import {Link as GatsbyLink} from 'gatsby';

export const MEDIA_SIZES = {
  xs: 360,
  sm: 576,
  md: 768,
  lg: 992,
  xl: 1200,
};

export const MEDIA = Object.keys(MEDIA_SIZES).reduce((acc, mediaSize) => {
  acc[mediaSize] = `@media (min-width: ${MEDIA_SIZES[mediaSize]}px)`;
  return acc;
}, {});

export const Center = styled.div`
  text-align: center;
`;

export const Container = styled.div`
  position: relative;
  max-width: 940px;
  padding: 0 ${props => props.mobilePadding || '16'}px;
  margin: 0 auto;

  ${MEDIA.sm} {
    padding: 0 25px;
  }
  ${MEDIA.md} {
    padding: 0 60px;
  }
  ${MEDIA.lg} {
    padding: 0 80px;
  }
`;

export const Row = styled(({spacing, ...rest}) => <div {...rest} />)`
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  margin: 0 -${props => props.spacing || '8'}px;
`;

export const Col = styled(({base, xs, sm, md, lg, xl, spacing, ...rest}) => (
  <div {...rest} />
))`
  flex: 1;
  padding: 0 ${props => props.spacing || '8'}px;
  ${props =>
    props.base &&
    css`
      flex-basis: ${props => (100 * props.base) / 12}%;
    `}
  ${props =>
    ['xs', 'sm', 'md', 'lg', 'xl']
      .filter(size => props[size])
      .map(
        size => css`
          ${MEDIA[size]} {
            flex-basis: ${props => (100 * props[size]) / 12}%;
          }
        `,
      )};
`;

export const Link = styled(GatsbyLink).attrs(() => ({
  activeStyle: {
    fontWeight: '600',
    background: 'white',
    color: '#7761d1',
  },
}))`
  color: inherit;
  text-decoration: none;
  transition: color 0.15s;
`;

export const ExternalLink = styled.a.attrs(() => ({
  target: '_blank',
  rel: 'noopener noreferrer',
}))`
  color: inherit;
  text-decoration: none;
  transition: color 0.15s;

  &:hover {
    color: #2263e5;
  }
`;

export const Flex = styled.div`
  display: flex;
  flex-wrap: wrap;
  justify-content: ${props => props.justify || 'space-between'};
  align-items: ${props => props.align || 'center'};

  > div {
    margin-right: 15px;
    margin-bottom: 15px;
    flex: ${props => props.type === 'even' && 1};
  }
`;

export const Button = styled.button`
  border: none;
  background: white;
  color: #5183f5;
  border: 2px dashed #5183f5;
  will-change: opacity;
  font-size: 15px;
  font-weight: 600;
  padding: 8px 16px;
  border-radius: 4px;
  margin: ${props => (props.marginless ? '0' : '0 8px 8px 0')};
  transition: background 0.2s, color 0.1s;

  &:hover {
    background: #5183f5;
    color: white;
  }
`;

export const Demo = styled.div`
  background: #eeeefa;
  margin: 15px -16px 25px;
  padding: 25px 16px 16px;

  ${MEDIA.sm} {
    padding-left: 25px;
    padding-right: 25px;
    margin-left: -25px;
    margin-right: -25px;
  }

  ${MEDIA.md} {
    border-radius: 8px;
  }
`;
