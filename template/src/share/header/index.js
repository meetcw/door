import './index.less'

export default {
    init: function () {
        document.querySelector('.header-wrapper .toggle').onclick = function () {
            document.querySelector('.header-wrapper').classList.toggle('with-menu')
        }
    }
}