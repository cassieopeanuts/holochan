<script>
    export let thread;
  </script>
  
  <div>
    <h4>{thread.title}</h4>
    <p>{thread.content}</p>
    <small>Posted on {new Date(thread.timestamp).toLocaleString()}</small>
  </div>
  