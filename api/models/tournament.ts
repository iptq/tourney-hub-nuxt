import { Column, Entity, BaseEntity, PrimaryGeneratedColumn } from "typeorm";

@Entity()
export class Tournament extends BaseEntity {
  @PrimaryGeneratedColumn()
  public id!: number;

  @Column()
  public name!: string;

  @Column()
  public format!: string;

  @Column()
  public modeBitflags!: number;
}
