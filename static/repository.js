Vue.use(VueRouter)

const Profile = Vue.component('profile', {
	template: `
<h1> PROFILE </h1>
`
})

const Message = Vue.component('message', {
	template: ``,
})

const ReadMe = Vue.component('readme', {
	template: `
<div id="readme">
	<div class="container" v-html="compiledMarkdown"></div>
</div>
	`,
	data () {
		return {
			contents: '# Empty README.md for this project'
		}
	},
	computed: {
		compiledMarkdown () {
			return marked(this.contents, {sanitize: true})
		}
	},
	methods: {

	}
})

const Repository = Vue.component('repository', {
	data () {
		return {
			githubAPI: {
				base_url: '',
			},
			gitlabAPIurl: {
				base_url: '',
			},
			github_projects: [],
			gitlab_projects: [],
			github_repositories: [],
		}
	},
	template: `
<div id="repository">
  <div class="container">
	<h1 class="page-header"> Github Projects </h1>
	<div class="card w-50" v-for="project in github_projects">
		<div class="card-body">
			<h3>{{project.name}}</h3>
			<p><a :href="project.html_url">Link to Repository</a></p>
			<readme v-href="compiledMarkdown"></readme>
		</div>
	</div>
  </div>
  <div class="container">
	<h1 class="page-header"> GitLab Projects </h1>
	<div class="card w-50" v-for="project in gitlab_projects">
		<div class="card-body">
			<h3>{{project.name}}</h3>
			<p><a v-bind:href="project.http_url_to_repo">Link to Repository</a></p>
		</div>
	</div>
  </div>
</div>
	`,
	computed: {

	},
	methods: {
		fetchGithubProjects() {
			const truth = axios
			  .get('https://api.github.com/users/nehri97/repos')
			  .then(response => (this.github_projects = response.data));
			return truth;
		},
		fetchGitLabProjects() {
			axios
			  .get('https://gitlab.com/api/v4/users/hfrancoi/projects')
			  .then(response => (this.gitlab_projects = response.data))
		},
		fetchAllProjects() {
			this.fetchGithubProjects();
			this.fetchGitLabProjects();
		},
		GithubReadmeContents(){
			console.log(this.github_projects)
			axios
			.get('https://api.github.com/repos/nehri97/learning-rust/readme')
			.then(response => console.log(atob(response.data.content)))
		}
	},
	created() {
		return this.fetchAllProjects();
	},
	mounted() {
		return this.GithubReadmeContents();
	}
})


const router = new VueRouter({
	routes: [
		{path: '/', component: Repository},
		{path: '/profile', component: Profile},
		{path: '/readme', component: ReadMe}
	]
})

var app = new Vue({
	router,
	el: '#app',
	template: `
<div>
	<ul class="nav justify-content-end">
	  <li class="nav-item">
	  	<a class="nav-link">
	    <router-link to="/profile">Profile</router-link>
	    </a>
	  </li>
	  <li class="nav-item">
	  	<a class="nav-link">
	    <router-link to="/">Projects</router-link>
	    </a>
	  </li>
	  <li class="nav-item">
	  	<a class="nav-link">
	    <router-link to="/readme">README</router-link>
	    </a>
	  </li>	  
	</ul>
	<router-view></router-view>
</div>
`
});
