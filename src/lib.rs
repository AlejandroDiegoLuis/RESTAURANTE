use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod pedidos_restaurante {
    use super::*;

    // CREATE
    pub fn registrar_pedido(
        ctx: Context<RegistrarPedido>,
        platillo: String,
        mesa: String,
        cliente: String,
    ) -> Result<()> {
        let pedido = &mut ctx.accounts.pedido;

        pedido.autoridad = ctx.accounts.autoridad.key();
        pedido.platillo = platillo;
        pedido.mesa = mesa;
        pedido.cliente = cliente;

        msg!("Pedido creado correctamente");
        Ok(())
    }

    // UPDATE
    pub fn actualizar_cliente(ctx: Context<ActualizarPedido>, nuevo_cliente: String) -> Result<()> {
        ctx.accounts.pedido.cliente = nuevo_cliente;
        msg!("Cliente actualizado");
        Ok(())
    }

    // READ
    pub fn ver_pedido(ctx: Context<VerPedido>) -> Result<()> {
        let p = &ctx.accounts.pedido;

        msg!("Platillo: {}", p.platillo);
        msg!("Mesa: {}", p.mesa);
        msg!("Cliente: {}", p.cliente);

        Ok(())
    }

    // DELETE
    pub fn eliminar_pedido(_ctx: Context<EliminarPedido>) -> Result<()> {
        msg!("Pedido eliminado");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(platillo: String)]
pub struct RegistrarPedido<'info> {
    #[account(
        init,
        payer = autoridad,
        seeds = [
            b"pedido",
            autoridad.key().as_ref(),
            platillo.as_bytes()
        ],
        bump,
        space = 8 + 32 + 4 + 50 + 4 + 50 + 4 + 50
    )]
    pub pedido: Account<'info, Pedido>,

    #[account(mut)]
    pub autoridad: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(platillo: String)]
pub struct ActualizarPedido<'info> {
    #[account(
        mut,
        seeds = [
            b"pedido",
            autoridad.key().as_ref(),
            platillo.as_bytes()
        ],
        bump,
        has_one = autoridad
    )]
    pub pedido: Account<'info, Pedido>,

    pub autoridad: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(platillo: String)]
pub struct VerPedido<'info> {
    #[account(
        seeds = [
            b"pedido",
            autoridad.key().as_ref(),
            platillo.as_bytes()
        ],
        bump
    )]
    pub pedido: Account<'info, Pedido>,

    pub autoridad: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(platillo: String)]
pub struct EliminarPedido<'info> {
    #[account(
        mut,
        close = autoridad,
        seeds = [
            b"pedido",
            autoridad.key().as_ref(),
            platillo.as_bytes()
        ],
        bump,
        has_one = autoridad
    )]
    pub pedido: Account<'info, Pedido>,

    #[account(mut)]
    pub autoridad: Signer<'info>,
}

#[account]
pub struct Pedido {
    pub autoridad: Pubkey,
    pub platillo: String,
    pub mesa: String,
    pub cliente: String,
}
