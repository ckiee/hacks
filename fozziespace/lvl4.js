var pageSizeForMobile = Math.random().toString();
document.cookie = "pagesize=" + pageSizeForMobile + ";";
window.vueInstances.homeContainer = new Vue({
	el: "#homeContainerVue",
	data: {
		success: false,
		successlvl: false,
		error: false,
	},
	methods: {
		setSize: function setSize() {
			var _this = this;

			var pageSize = "";
			var decoded = decodeURIComponent(document.cookie);
			var name = "pagesize=";
			var ca = decoded.split(";");
			for (var i = 0; i < ca.length; i++) {
				var c = ca[i];
				while (c.charAt(0) === " ") {
					c = c.substring(1);
				}
				if (c.indexOf(name) === 0) {
					pageSize = c.substring(name.length, c.length);
				}
			}
			if (this.$refs.levelpass.value === pageSize) {
				this.success = true;
				this.successlvl = true;
				setTimeout(function () {
					_this.success = false;
				}, 2500);
			} else {
				this.error = true;
				setTimeout(function () {
					_this.error = false;
				}, 1100);
			}
		},
	},
});
