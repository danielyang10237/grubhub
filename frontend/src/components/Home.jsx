import { React } from 'react';
import TopBar from './TopBar';

const Home = () => {
    return (<header className="App-header">
    <TopBar />
    <img
      style={{ opacity: "0.4" }}
      className="background-image"
      src="https://news.stanford.edu/report/wp-content/uploads/sites/3/2022/10/ssa_100622_0262-1.jpg"
      alt="background image"
    ></img>
    <div className="title-container">
      <h2 className="title1">Hey,</h2>
      <h1 className="title2">Stanford</h1>
      <h3 className="title3">Learn something new.</h3>
      <h3 className="title3">Find community on-campus.</h3>
    </div>
    <img
      className="red-wave"
      src={`${process.env.PUBLIC_URL}/resources/wave.png`}
      alt="wave"
    ></img>
  </header>);
}

export default Home;