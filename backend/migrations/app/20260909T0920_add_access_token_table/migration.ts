#!/usr/bin/env -S node
import type { Contract as End } from '../../snapshots/b0d4fe8ebe2c46d4f09f69363b4dd9e9c4eb18d09491ae7564c9d8d0ffada9de/contract';
import endContract from '../../snapshots/b0d4fe8ebe2c46d4f09f69363b4dd9e9c4eb18d09491ae7564c9d8d0ffada9de/contract.json' with { type: 'json' };
import { Migration, MigrationCLI, col, primaryKey } from '@prisma/orm-postgres/migration';

export default class M extends Migration<never, End> {
  override readonly endContractJson = endContract;

  override get operations() {
    return [
      this.createSchema({ schema: 'public' }),
      this.createTable({
        schema: 'public',
        table: 'accessToken',
        columns: [
          col('created_at', 'timestamptz', {
            notNull: true,
            codecRef: { codecId: 'pg/timestamptz-temporal@1' },
          }),
          col('id', 'SERIAL', { notNull: true, codecRef: { codecId: 'pg/int4@1' } }),
          col('token', 'text', { notNull: true, codecRef: { codecId: 'pg/text@1' } }),
        ],
        constraints: [primaryKey(['id'])],
      }),
      this.addUnique({
        schema: 'public',
        table: 'accessToken',
        constraint: 'accessToken_token_key',
        columns: ['token'],
      }),
    ];
  }
}

MigrationCLI.run(import.meta.url, M);
