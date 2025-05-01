declare global {
  interface Date {
    getJD(): number;
    setJD(jd: number): void;
    getMJD(): number;
    setMJD(mjd: number): void;
  }
}

export { };
