window.vueInstances.homeContainer = new Vue({
	el: "#homeContainerVue",
	data: {
		success: false,
		successlvl: false,
		error: false,
	},
	methods: {
		checkpass: function checkpass() {
			var _this = this;

			var _0x5ead = [
				"value",
				"iOS",
				"success",
				"successlvl",
				"$refs",
				"levelpass",
			];
			(function (arr, _0x5a838c) {
				var reverseo = function _0x38c0be(_0x57d08d) {
					while (--_0x57d08d) {
						arr["push"](arr["shift"]());
					}
				};
				reverseo(++_0x5a838c);
			})(_0x5ead, 0x106);
			//[ '$refs', 'levelpass', 'value', 'iOS', 'success', 'successlvl' ]
			var getFromArr = function _0x3196(_0x548a4d, _0x5ba61d) {
				_0x548a4d = _0x548a4d - 0x0;
				var _0x32c05c = _0x5ead[_0x548a4d];
				return _0x32c05c;
			};
			if (
				this[getFromArr("0x0")][getFromArr("0x1") /*iOS*/][
					getFromArr("0x2") /*success*/
				] == getFromArr("0x3")
			) {
				this[getFromArr("0x4")] = !![];
				this[getFromArr("0x5")] = !![];
				setTimeout(function () {
					_this[getFromArr("0x4")] = ![];
				}, 0x9c4);
			} else {
				this["error"] = !![];
				setTimeout(function () {
					_this["error"] = ![];
				}, 0x44c);
			}
		},
	},
});
