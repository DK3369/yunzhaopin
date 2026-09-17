import { serveSiteCss } from '../../utils/serveLegacyCss'

export default defineEventHandler((event) => serveSiteCss(event, 'h5'))
