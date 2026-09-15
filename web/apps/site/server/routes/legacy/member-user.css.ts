import { serveLegacyCss } from '../../utils/serveLegacyCss'

export default defineEventHandler((event) => serveLegacyCss(event, 'pc-user'))
