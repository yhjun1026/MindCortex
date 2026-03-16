export interface Theme {
  name: string;
  mode: 'light' | 'dark' | 'auto';
  colors: ThemeColors;
}

export interface ThemeColors {
  primary: string;
  secondary: string;
  background: string;
  foreground: string;
  surface: string;
  text: string;
  textSecondary: string;
  border: string;
  success: string;
  warning: string;
  error: string;
}

// 浅色主题
export const lightTheme: Theme = {
  name: 'Light',
  mode: 'light',
  colors: {
    primary: '#4a90e2',
    secondary: '#764ba2',
    background: '#ffffff',
    foreground: '#f9f9f9',
    surface: '#f5f5f5',
    text: '#333333',
    textSecondary: '#666666',
    border: '#e0e0e0',
    success: '#4caf50',
    warning: '#ff9800',
    error: '#f44336',
  },
};

// 深色主题
export const darkTheme: Theme = {
  name: 'Dark',
  mode: 'dark',
  colors: {
    primary: '#5aaaf7',
    secondary: '#9d6eb8',
    background: '#1a1a1a',
    foreground: '#2d2d2d',
    surface: '#3d3d3d',
    text: '#ffffff',
    textSecondary: '#b0b0b0',
    border: '#404040',
    success: '#66bb6a',
    warning: '#ffb74d',
    error: '#ef5350',
  },
};

export const themes = {
  light: lightTheme,
  dark: darkTheme,
};
