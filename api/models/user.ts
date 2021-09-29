import { Column, Entity, BaseEntity, PrimaryGeneratedColumn } from "typeorm";

export class ExternalUser {
  @Column({ nullable: true })
  public userId!: string;

  @Column({ nullable: true })
  public username!: string;

  @Column({ nullable: true })
  public accessToken?: string;
}

@Entity()
export class User extends BaseEntity {
  @PrimaryGeneratedColumn()
  public id!: number;

  @Column(() => ExternalUser)
  public osu!: ExternalUser;
}
