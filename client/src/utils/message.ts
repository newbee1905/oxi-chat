export interface Message {
	message: string;
	// optional id and username 
	// if id or username is null
	// -> current user
	id?: string;
	username?: string;
}