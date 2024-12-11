'use client'

import { getSolKratusProgram, getSolKratusProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

export function useSolKratusProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getSolKratusProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getSolKratusProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['SolKratus', 'all', { cluster }],
    queryFn: () => program.account.SolKratus.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['SolKratus', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ SolKratus: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useSolKratusProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useSolKratusProgram()

  const accountQuery = useQuery({
    queryKey: ['SolKratus', 'fetch', { cluster, account }],
    queryFn: () => program.account.SolKratus.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['SolKratus', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ SolKratus: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['SolKratus', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ SolKratus: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['SolKratus', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ SolKratus: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['SolKratus', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ SolKratus: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
