import {MigrationInterface, QueryRunner} from "typeorm";

export class initial1632892190953 implements MigrationInterface {
    name = 'initial1632892190953'

    public async up(queryRunner: QueryRunner): Promise<void> {
        await queryRunner.query(`CREATE TABLE "tournament" ("id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, "name" varchar NOT NULL, "format" varchar NOT NULL, "modeBitflags" integer NOT NULL)`);
        await queryRunner.query(`CREATE TABLE "user" ("id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, "osuUserid" varchar, "osuUsername" varchar, "osuAccesstoken" varchar)`);
    }

    public async down(queryRunner: QueryRunner): Promise<void> {
        await queryRunner.query(`DROP TABLE "user"`);
        await queryRunner.query(`DROP TABLE "tournament"`);
    }

}
