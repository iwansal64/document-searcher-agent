#!/usr/bin/env -S node
import type { Contract as Start } from '../../snapshots/b0d4fe8ebe2c46d4f09f69363b4dd9e9c4eb18d09491ae7564c9d8d0ffada9de/contract';
import startContract from '../../snapshots/b0d4fe8ebe2c46d4f09f69363b4dd9e9c4eb18d09491ae7564c9d8d0ffada9de/contract.json' with { type: 'json' };
import type { Contract as End } from '../../snapshots/eeea1bc0240840554b846c4015915906a576177263982d8c3ee3636dc03a1204/contract';
import endContract from '../../snapshots/eeea1bc0240840554b846c4015915906a576177263982d8c3ee3636dc03a1204/contract.json' with { type: 'json' };
import { Migration, MigrationCLI } from '@prisma/orm-postgres/migration';

export default class M extends Migration<Start, End> {
  override readonly startContractJson = startContract;
  override readonly endContractJson = endContract;

  override get operations() {
    return [
      this.setDefault({
        schema: 'public',
        table: 'accessToken',
        column: 'created_at',
        defaultSql: 'DEFAULT (now())',
      }),
    ];
  }
}

MigrationCLI.run(import.meta.url, M);
