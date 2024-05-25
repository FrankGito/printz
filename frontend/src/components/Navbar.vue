<template>
  <nav class="navbar navbar-expand-lg bg-light" data-bs-theme="light">
    <div class="container-fluid">
      <RouterLink class="navbar-brand" to="/">
      Printz
      </RouterLink>
      <button
        class="navbar-toggler collapsed"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbarColor03"
        aria-controls="navbarColor03"
        aria-expanded="false"
        aria-label="Toggle navigation"
      >
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="navbar-collapse collapse" id="navbarColor03" style="">
        <ul class="navbar-nav me-auto">
          <li class="nav-item">
            <RouterLink class="nav-link active" to="/"
              >Home
              <span class="visually-hidden">(current)</span>
            </RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink class="nav-link" to="/offers">Offers</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink class="nav-link" to="/about">About</RouterLink>
            <!-- <a class="nav-link" href="#">About</a> -->
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>
